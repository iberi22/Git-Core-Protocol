import os.path
import base64
import re
import logging
import subprocess
from google.auth.transport.requests import Request
from google.oauth2.credentials import Credentials
from google_auth_oauthlib.flow import InstalledAppFlow
from googleapiclient.discovery import build
from googleapiclient.errors import HttpError
from bs4 import BeautifulSoup

# Configuración de Logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')

# Scopes requeridos para leer y modificar correos
SCOPES = ['https://www.googleapis.com/auth/gmail.modify']

# Obtener directorio del script
SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
TOKEN_FILE = os.path.join(SCRIPT_DIR, '..', 'token.json')
CREDENTIALS_FILE = os.path.join(SCRIPT_DIR, '..', 'credentials.json')

def get_gmail_service():
    """Autenticación mejorada con fallbacks múltiples."""
    creds = None

    # 1. Intentar cargar token.json (sesión guardada)
    if os.path.exists(TOKEN_FILE):
        try:
            creds = Credentials.from_authorized_user_file(TOKEN_FILE, SCOPES)
            if creds.valid:
                logging.info("✅ Usando credenciales guardadas en token.json")
                try:
                    service = build('gmail', 'v1', credentials=creds)
                    return service
                except Exception as e:
                    logging.warning(f"Error al conectar con token.json: {e}")
                    creds = None
        except Exception:
            logging.warning("token.json inválido, se intentará re-autenticar.")
            creds = None

    # 2. Si el token expiró, intentar refrescarlo
    if creds and creds.expired and creds.refresh_token:
        try:
            logging.info("🔄 Refrescando token expirado...")
            creds.refresh(Request())
            with open(TOKEN_FILE, 'w') as token:
                token.write(creds.to_json())
            service = build('gmail', 'v1', credentials=creds)
            return service
        except Exception as e:
            logging.warning(f"No se pudo refrescar el token: {e}")
            creds = None

    # 3. Intentar flujo OAuth con credentials.json
    if os.path.exists(CREDENTIALS_FILE):
        try:
            logging.info("🔐 Iniciando flujo OAuth local con credentials.json...")
            flow = InstalledAppFlow.from_client_secrets_file(
                CREDENTIALS_FILE, SCOPES)
            creds = flow.run_local_server(port=0)

            # Guardar token para la próxima vez
            with open(TOKEN_FILE, 'w') as token:
                token.write(creds.to_json())

            logging.info("✅ Autenticación exitosa!")
            service = build('gmail', 'v1', credentials=creds)
            return service
        except Exception as e:
            logging.error(f"Error con credentials.json: {e}")

    # 4. Si todo falló, mostrar instrucciones claras
    logging.error("❌ No se encontraron credenciales válidas.")
    logging.info("\n📋 INSTRUCCIONES:")
    logging.info("1. Ve a: https://console.cloud.google.com/")
    logging.info("2. Crea un proyecto o usa 'saber-proactivo-2025'")
    logging.info("3. Habilita Gmail API")
    logging.info("4. Ve a 'Credenciales' → 'Crear Credenciales' → 'ID de cliente OAuth'")
    logging.info("5. Configura la pantalla de consentimiento (tipo: Externa)")
    logging.info("6. Descarga el JSON y guárdalo como: tools/email-handler/credentials.json")
    logging.info("7. Vuelve a ejecutar este script.\n")

    return None

def parse_github_email(snippet, body):
    """
    Intenta extraer información relevante del correo de GitHub.
    Busca patrones como 'Run failed: CI - main' y el nombre del repo.
    """
    # Ejemplo de asunto/snippet: "[iberi22/domus-otec] Run failed: CI - main (c33f718)"

    # Regex para capturar repo y workflow
    # Patrón: [owner/repo] Run failed: Workflow Name - branch (commit)
    match = re.search(r'\[([\w-]+/[\w-]+)\] Run failed: (.+?) -', snippet)

    if match:
        repo = match.group(1)
        workflow = match.group(2)
        logging.info(f"Detectado fallo en Repo: {repo}, Workflow: {workflow}")
        return {
            'type': 'github_action_failure',
            'repo': repo,
            'workflow': workflow,
            'snippet': snippet
        }

    return None

def process_messages(service):
    """Busca y procesa correos de notificaciones de GitHub."""
    try:
        # Filtrar correos no leídos de notificaciones de GitHub
        # query = 'from:notifications@github.com is:unread subject:"Run failed"'
        # Para pruebas, podemos ser más amplios o específicos
        query = 'from:notifications@github.com is:unread "Run failed"'

        results = service.users().messages().list(userId='me', q=query).execute()
        messages = results.get('messages', [])

        if not messages:
            logging.info('No se encontraron notificaciones de fallos nuevas.')
            return

        logging.info(f'Se encontraron {len(messages)} correos de fallos.')

        for message in messages:
            msg = service.users().messages().get(userId='me', id=message['id']).execute()

            snippet = msg.get('snippet', '')
            payload = msg.get('payload', {})
            headers = payload.get('headers', [])

            subject = next((h['value'] for h in headers if h['name'] == 'Subject'), 'Sin Asunto')
            logging.info(f"Procesando correo: {subject}")

            # Analizar contenido
            issue_data = parse_github_email(subject + " " + snippet, "")

            if issue_data:
                # Verificar si el workflow ya está pasando
                is_fixed = check_workflow_status(issue_data['repo'], issue_data['workflow'])

                if is_fixed:
                    logging.info(f"✅ Workflow {issue_data['workflow']} ya está PASANDO. Archivando correo...")
                    archive_message(service, message['id'])
                else:
                    # AQUÍ IRÍA LA LÓGICA DE REPARACIÓN
                    # 1. Verificar logs con `gh run view ...`
                    # 2. Intentar rerun o crear issue
                    logging.info(f"⚠️ Acción requerida para {issue_data['repo']}")

                    # TODO: Implementar llamada a GH CLI
                    # subprocess.run(["gh", "run", "rerun", ...])

                    # Por ahora, marcar como leído para no procesarlo múltiples veces
                    mark_as_read(service, message['id'])
            else:
                logging.info("No se pudo extraer información estructurada del correo.")
                mark_as_read(service, message['id'])

    except HttpError as error:
        logging.error(f'Ocurrió un error al procesar mensajes: {error}')

def mark_as_read(service, msg_id):
    """Marca un correo como leído (remueve la etiqueta UNREAD)."""
    try:
        service.users().messages().modify(userId='me', id=msg_id, body={
            'removeLabelIds': ['UNREAD']
        }).execute()
        logging.info(f"✅ Mensaje {msg_id} marcado como leído.")
    except HttpError as error:
        logging.error(f'Error al marcar mensaje como leído: {error}')

def archive_message(service, msg_id):
    """Archiva un correo (remueve de INBOX)."""
    try:
        service.users().messages().modify(userId='me', id=msg_id, body={
            'removeLabelIds': ['INBOX']
        }).execute()
        logging.info(f"📦 Mensaje {msg_id} archivado.")
    except HttpError as error:
        logging.error(f'Error al archivar mensaje: {error}')

def check_workflow_status(repo, workflow_name):
    """Verifica si el workflow más reciente de un repo está pasando."""
    try:
        # Ejecutar gh CLI para verificar el último run
        result = subprocess.run(
            ['gh', 'run', 'list', '--repo', repo, '--workflow', workflow_name,
             '--limit', '1', '--json', 'conclusion'],
            capture_output=True,
            text=True
        )

        if result.returncode == 0:
            import json
            runs = json.loads(result.stdout)
            if runs and len(runs) > 0:
                return runs[0].get('conclusion') == 'success'
        return False
    except Exception as e:
        logging.error(f"Error verificando workflow status: {e}")
        return False

if __name__ == '__main__':
    import argparse

    parser = argparse.ArgumentParser(description='GitHub Email Handler Agent')
    parser.add_argument('--max-emails', type=int, default=50, help='Maximum emails to process')
    parser.add_argument('--dry-run', action='store_true', help='Dry run mode (no modifications)')
    args = parser.parse_args()

    logging.info(f"Iniciando Email Handler Agent (max: {args.max_emails} emails)...")

    if args.dry_run:
        logging.info("🔍 DRY RUN MODE - No se modificarán correos")

    service = get_gmail_service()
    if service:
        process_messages(service)
