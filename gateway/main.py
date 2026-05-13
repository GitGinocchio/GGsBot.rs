import os
import stat
import requests
import subprocess
import sys

REPO_OWNER = "GitGinocchio"
REPO_NAME = "discord-ws-http-bridge"
BINARY_NAME = "discord-ws-http-bridge"
VERSION_FILE = ".version"

def get_local_version():
    if os.path.exists(VERSION_FILE):
        with open(VERSION_FILE, 'r') as f:
            return f.read().strip()
    return None

def save_local_version(sha):
    with open(VERSION_FILE, 'w') as f:
        f.write(sha)

def download_latest_binary():
    print(f"Controllo aggiornamenti su GitHub...")
    api_url = f"https://api.github.com/repos/{REPO_OWNER}/{REPO_NAME}/releases/latest"
    
    try:
        response = requests.get(api_url)
        response.raise_for_status()
        release_data = response.json()
        remote_hash = release_data['tag_name']
        
        local_hash = get_local_version()

        if local_hash == remote_hash and os.path.exists(BINARY_NAME):
            print(f"Il bot è già aggiornato (Versione: {local_hash}).")
            return True

        print(f"Nuova versione trovata: {remote_hash} (Locale: {local_hash or 'nessuna'})")

        download_url = next((
            a['browser_download_url'] for a in release_data['assets'] 
            if a['name'] == BINARY_NAME), 
            None
        )
        
        if not download_url:
            print(f"Errore: l'eseguibile {BINARY_NAME} non trovato nella release.")
            return False

        print(f"Scaricamento in corso...")
        with requests.get(download_url, stream=True) as r:
            r.raise_for_status()
            with open(BINARY_NAME, 'wb') as f:
                for chunk in r.iter_content(chunk_size=8192):
                    f.write(chunk)

        st = os.stat(BINARY_NAME)
        os.chmod(BINARY_NAME, st.st_mode | stat.S_IEXEC)
        save_local_version(remote_hash)
        
        print(f"Aggiornamento completato: {remote_hash}")
        return True

    except Exception as e:
        print(f"Errore durante il controllo aggiornamenti: {e}")
        return os.path.exists(BINARY_NAME)

def run():
    print(f"Avvio di {BINARY_NAME}...")
    try:
        subprocess.run([f"./{BINARY_NAME}"], check=True)
    except KeyboardInterrupt:
        print("\nSpegnimento manuale.")
    except Exception as e:
        print(f"Il bridge si è interrotto: {e}")

if __name__ == "__main__":
    if download_latest_binary():
        run()
    else:
        print("Impossibile avviare: binario mancante e aggiornamento fallito.")
        sys.exit(1)