
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

curl -fsSL https://nvidia.github.io/libnvidia-container/gpgkey | sudo gpg --dearmor -o /usr/share/keyrings/nvidia-container-toolkit-keyring.gpg
curl -fsSL https://nvidia.github.io/libnvidia-container/stable/deb/nvidia-container-toolkit.list \
  | sed 's#deb https://#deb [signed-by=/usr/share/keyrings/nvidia-container-toolkit-keyring.gpg] https://#' \
  | sudo tee /etc/apt/sources.list.d/nvidia-container-toolkit.list

sudo apt-get update
sudo apt-get install -y nvidia-container-toolkit

sudo nvidia-ctk runtime configure
sudo systemctl restart docker


docker run --runtime=nvidia \
  -e NVIDIA_VISIBLE_DEVICES=all \
  -e NVIDIA_DRIVER_CAPABILITIES=compute,utility \
  -e TORCH_DEVICE=cuda \
  -e OUTPUT_DIR=/data/marker_out \
  -e HF_HOME=/root/.cache/huggingface \
  -e TRANSFORMERS_CACHE=/root/.cache/huggingface \
  -e HUGGINGFACE_HUB_CACHE=/root/.cache/huggingface \
  -v /data/marker_out:/data/marker_out \
  -v /data/marker_cache/datalab:/root/.cache/datalab \
  -v /data/marker_cache/hf:/root/.cache/huggingface \
  -v /data/marker_cache/torch:/root/.cache/torch \
  -p 8000:8080 \
  marker-server



```