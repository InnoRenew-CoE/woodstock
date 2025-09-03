FROM python:3.11-slim

# System deps for torch and image libs
RUN apt-get update && apt-get install -y --no-install-recommends \
    libglib2.0-0 libsm6 libxext6 libxrender1 \
    libgl1 libjpeg62-turbo zlib1g \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY requirements.txt .
RUN pip install -U pip && pip install -r requirements.txt

COPY server.py .

ENV PORT=8080
EXPOSE 8080
CMD ["python", "server.py"]
