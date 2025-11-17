import os
import glob
import subprocess
from urllib.parse import urlparse

import zstandard as zstd
from dotenv import load_dotenv

# Load environment variables from .env file
load_dotenv()

# Get the PostgreSQL database URL from the .env file
DATABASE_URL = os.getenv('DATABASE_URL')

# List of possible translations
translations = ["TOVBSI", "KJV", "MLSVP", "ASV", "WEB", "WEBU", "GOVBSI", "OOVBSI"]

def dump_to_csv(translation):
    if translation not in translations:
        print(f"Invalid translation: {translation}")
        return

    # Extract connection parameters from the DATABASE_URL
    parsed_url = urlparse(DATABASE_URL)
    host = parsed_url.hostname
    port = parsed_url.port
    user = parsed_url.username
    password = parsed_url.password
    dbname = parsed_url.path.lstrip('/')

    # Prepare the psql command using \copy (client-side COPY)
    query = f"\\copy (SELECT * FROM fulltable WHERE translation='{translation}') TO 'csv/{translation}.csv' WITH CSV HEADER;"

    # Run the psql command to export the data to a CSV file
    try:
        result = subprocess.run(
            ['psql', 
             f'postgresql://{user}:{password}@{host}:{port}/{dbname}', 
             '-c', query],
            check=True,
            text=True,
            capture_output=True
        )
        print(f"Data exported to {translation}.csv successfully.")
    except subprocess.CalledProcessError as e:
        print(f"Error: {e.stderr}")

def compress_csv_files(directory):
    # Globbing all CSV files in the given directory
    csv_files = glob.glob(os.path.join(directory, "*.csv"))
    
    # Compress each CSV file
    for csv_file in csv_files:
        # Create the output filename with the .csv.zst extension
        output_file = csv_file + ".zst"
        
        try:
            with open(csv_file, 'rb') as f_in:
                with open(output_file, 'wb') as f_out:
                    # Create a Zstandard compressor object
                    compressor = zstd.ZstdCompressor()
                    # Compress the file
                    compressor.copy_stream(f_in, f_out)
                    print(f"Compressed {csv_file} to {output_file}")
        except Exception as e:
            print(f"Error compressing {csv_file}: {e}")

def delete_csv_files(directory):
    # Globbing all CSV files in the given directory
    csv_files = glob.glob(os.path.join(directory, "*.csv"))
    
    # Deleting each CSV file
    for csv_file in csv_files:
        try:
            os.remove(csv_file)
            print(f"Deleted {csv_file}")
        except Exception as e:
            print(f"Error deleting {csv_file}: {e}")


for tr in translations:
    dump_to_csv(tr)

compress_csv_files("csv")
delete_csv_files("csv")
