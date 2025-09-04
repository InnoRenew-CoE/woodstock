import os
import uuid
import base64
from io import BytesIO
from typing import List, Dict, Any

from fastapi import FastAPI, UploadFile, File, Form, HTTPException, Depends, Header
from fastapi.responses import JSONResponse
from fastapi.encoders import jsonable_encoder
from pydantic import BaseModel
import uvicorn

from marker.converters.pdf import PdfConverter
from marker.models import create_model_dict
from marker.output import text_from_rendered
from marker.config.parser import ConfigParser
from tempfile import NamedTemporaryFile

ADMIN_TOKEN = os.getenv("ADMIN_TOKEN")
print(f"Admin token set: {bool(ADMIN_TOKEN)}")

def require_token(authorization: str = Header(None)):
    if not ADMIN_TOKEN:
        return  # no protection if env not set
    if not authorization or not authorization.startswith("Bearer "):
        raise HTTPException(status_code=401, detail="Unauthorized")
    token = authorization.split(" ", 1)[1]
    if token != ADMIN_TOKEN:
        raise HTTPException(status_code=401, detail="Unauthorized")


app = FastAPI(title="Marker Server", version="1.0.0")

CHUNK_SIZE = 8 * 1024 * 1024  # 8 MB


class ConvertResponse(BaseModel):
    job_id: str
    outputs: Dict[str, Any]
    metadata: Dict[str, Any]


_CONVERTER_CACHE: Dict[tuple, PdfConverter] = {}


def make_converter(
    output_format: str,
    use_llm: bool,
    force_ocr: bool,
    paginate_output: bool,
    strip_existing_ocr: bool,
    redo_inline_math: bool,
) -> PdfConverter:
    key = (
        output_format,
        bool(use_llm),
        bool(force_ocr),
        bool(paginate_output),
        bool(strip_existing_ocr),
        bool(redo_inline_math),
    )
    if key in _CONVERTER_CACHE:
        return _CONVERTER_CACHE[key]

    cfg = {
        "output_format": output_format,
        "paginate_output": paginate_output,
        "force_ocr": force_ocr,
        "strip_existing_ocr": strip_existing_ocr,
        "redo_inline_math": redo_inline_math,
    }
    config_parser = ConfigParser(cfg)
    converter = PdfConverter(
        config=config_parser.generate_config_dict(),
        artifact_dict=create_model_dict(),
        processor_list=config_parser.get_processors(),
        renderer=config_parser.get_renderer(),
        llm_service=config_parser.get_llm_service() if use_llm else None,
    )
    _CONVERTER_CACHE[key] = converter
    return converter


def serialize_images(images) -> Dict[str, str]:
    out: Dict[str, str] = {}

    if isinstance(images, dict):
        items = images.items()
    else:
        items = [(f"image_{i+1}.png", im) for i, im in enumerate(images)]

    for name, obj in items:
        if isinstance(obj, (bytes, bytearray)):
            data_bytes = bytes(obj)
        else:
            try:
                buf = BytesIO()
                obj.save(buf, format="PNG")
                data_bytes = buf.getvalue()
            except Exception:
                continue
        out[name] = base64.b64encode(data_bytes).decode("ascii")
    return out


def to_plain(obj: Any) -> Any:
    # already plain
    if isinstance(obj, (dict, list, str, int, float, bool)) or obj is None:
        return obj
    # robust encoder that handles pydantic models and numpy types
    try:
        return jsonable_encoder(obj)
    except Exception:
        pass
    # last resort: try model_dump_json then parse
    try:
        if hasattr(obj, "model_dump_json"):
            import json
            return json.loads(obj.model_dump_json())
    except Exception:
        pass
    # fallback to string
    return str(obj)


@app.get("/health")
def health():
    device = os.getenv("TORCH_DEVICE", "auto")
    return {"status": "ok", "device": device}

@app.post("/convert", response_model=ConvertResponse, dependencies=[Depends(require_token)])
async def convert(
    file: UploadFile = File(...),
    formats: str = Form("markdown,json"),
    use_llm: bool = Form(False),
    force_ocr: bool = Form(False),
    paginate_output: bool = Form(False),
    strip_existing_ocr: bool = Form(False),
    redo_inline_math: bool = Form(False),
    return_images: bool = Form(False),
):
    print("Received convert request")

    requested: List[str] = [s.strip().lower() for s in formats.split(",") if s.strip()]
    allowed = {"markdown", "json", "html", "chunks"}
    if not requested or any(fmt not in allowed for fmt in requested):
        raise HTTPException(status_code=400, detail=f"formats must be subset of {sorted(list(allowed))}")

    max_mb = int(os.getenv("MAX_UPLOAD_MB", "2048"))
    chunk_size = int(os.getenv("UPLOAD_CHUNK_BYTES", str(8 * 1024 * 1024)))
    tmpdir = os.getenv("UPLOAD_TMPDIR", "/tmp")

    # stream upload to disk
    suffix = f"{uuid.uuid4()}_{file.filename or 'input.bin'}"
    tf = NamedTemporaryFile(delete=False, dir=tmpdir, suffix=suffix)
    in_path = tf.name
    total = 0
    try:
        while True:
            chunk = await file.read(chunk_size)
            if not chunk:
                break
            tf.write(chunk)
            total += len(chunk)
            if total > max_mb * 1024 * 1024:
                raise HTTPException(status_code=413, detail="file too large")
    finally:
        tf.close()

    job_id = str(uuid.uuid4())
    outputs: Dict[str, Any] = {}
    meta_all: Dict[str, Any] = {}

    try:
        for fmt in requested:
            converter = make_converter(
                output_format=fmt,
                use_llm=use_llm,
                force_ocr=force_ocr,
                paginate_output=paginate_output,
                strip_existing_ocr=strip_existing_ocr,
                redo_inline_math=redo_inline_math,
            )
            rendered = converter(in_path)

            if fmt == "markdown":
                text, metadata, images = text_from_rendered(rendered)
                outputs["markdown"] = text
                if return_images and images:
                    outputs["images"] = serialize_images(images)
                meta_all["markdown_metadata"] = metadata or {}

            elif fmt == "json":
                outputs["json"] = to_plain(rendered)
                meta_all["json_metadata"] = getattr(rendered, "metadata", {}) or {}

            elif fmt == "html":
                outputs["html"] = getattr(rendered, "html", "") or ""
                meta_all["html_metadata"] = getattr(rendered, "metadata", {}) or {}

            elif fmt == "chunks":
                outputs["chunks"] = to_plain(rendered)
                meta_all["chunks_metadata"] = getattr(rendered, "metadata", {}) or {}
    finally:
        try:
            os.remove(in_path)
        except Exception:
            pass

    return JSONResponse(
        ConvertResponse(
            job_id=job_id,
            outputs=outputs,
            metadata=meta_all,
        ).model_dump()
    )

if __name__ == "__main__":
    port = int(os.getenv("PORT", "8080"))
    workers = int(os.getenv("WORKERS", "1"))
    uvicorn.run("server:app", host="0.0.0.0", port=port, workers=workers, timeout_keep_alive=28800)
