
# run local

```
python -m venv .venv && . .venv/bin/activate
pip install -r requirements.txt
mkdir -p /data/marker_out
python server.py
```

# run docker

```
docker build -t marker-server .
docker run -d -p 8080:8080 -e OUTPUT_DIR=/data/marker_out -v /data/marker_out:/data/marker_out marker-server
```