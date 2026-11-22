import requests
import json
import re
import time
from pathlib import Path

OUTPUT_DIR = Path("pubchem_files")
OUTPUT_DIR.mkdir(exist_ok=True)

BASE_URL = (
    "https://pubchem.ncbi.nlm.nih.gov/"
    "rest/pug_view/data/compound/{}/JSON"
)

for cid in range(1, 201):
    try:
        response = requests.get(
            BASE_URL.format(cid),
            timeout=30
        )
        response.raise_for_status()

        data = response.json()

        record = data.get("Record", {})
        name = record.get("RecordTitle", f"CID_{cid}")

        # Make the name safe for filenames
        safe_name = re.sub(
            r'[<>:"/\\|?*]', "_", name
        ).strip()

        filename = (
            OUTPUT_DIR /
            f"pubchem_pug_view_{safe_name}.json"
        )

        with open(filename, "w", encoding="utf-8") as f:
            json.dump(data, f, indent=2, ensure_ascii=False)

        print(f"Downloaded CID {cid}: {filename.name}")

    except Exception as error:
        print(f"Error for CID {cid}: {error}")

    time.sleep(0.25)

print("Download process completed.")
