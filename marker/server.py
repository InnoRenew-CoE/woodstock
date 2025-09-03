import uuid
from typing import List, Dict, Any, Optional

from fastapi import FastAPI, UploadFile, File, Form, HTTPException
from fastapi.responses import JSONResponse
from pydantic import BaseModel

from marker.converters.pdf import PdfConverter
from marker.models import create_model_dict
from marker.output import text_from_rendered
from marker.config.parser import ConfigParser
import os
import uvicorn


app = FastAPI(title="Marker Server", version="1.0.0")

class ConvertResponse(BaseModel):
    job_id: str
    outputs: Dict[str, Any]
    metadata: Dict[str, Any]

def make_converter(output_format: str,
                   use_llm: bool,
                   force_ocr: bool,
                   paginate_output: bool,
                   strip_existing_ocr: bool,
                   redo_inline_math: bool) -> PdfConverter:
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
    return converter

@app.get("/health")
def health():
    return {"status": "ok"}

@app.post("/convert", response_model=ConvertResponse)
async def convert(
    file: UploadFile = File(...),
    formats: str = Form("markdown,json"),
    use_llm: bool = Form(False),
    force_ocr: bool = Form(False),
    paginate_output: bool = Form(False),
    strip_existing_ocr: bool = Form(False),
    redo_inline_math: bool = Form(False),
):
    # Parse formats
    requested: List[str] = [s.strip().lower() for s in formats.split(",") if s.strip()]
    allowed = {"markdown","json","html","chunks"}
    if not requested or any(fmt not in allowed for fmt in requested):
        raise HTTPException(status_code=400, detail=f"formats must be subset of {sorted(list(allowed))}")

    # Save upload temporarily in memory
    contents = await file.read()
    in_path = f"/tmp/{uuid.uuid4()}_{file.filename or 'input.pdf'}"
    with open(in_path, "wb") as f:
        f.write(contents)

    job_id = str(uuid.uuid4())
    outputs: Dict[str, Any] = {}
    meta_all: Dict[str, Any] = {}

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
            outputs["images"] = {name: data.decode("latin1") for name, data in images.items()} if images else {}
            meta_all["markdown_metadata"] = metadata or {}

        elif fmt == "json":
            outputs["json"] = rendered.model_dump()
            meta_all["json_metadata"] = getattr(rendered, "metadata", {}) or {}

        elif fmt == "html":
            outputs["html"] = getattr(rendered, "html", "")
            meta_all["html_metadata"] = getattr(rendered, "metadata", {}) or {}

        elif fmt == "chunks":
            outputs["chunks"] = rendered.model_dump()
            meta_all["chunks_metadata"] = getattr(rendered, "metadata", {}) or {}

    return JSONResponse(ConvertResponse(
        job_id=job_id,
        outputs=outputs,
        metadata=meta_all,
    ).model_dump())

if __name__ == "__main__":
    # Env PORT and WORKERS let you scale without CLI flags
    port = int(os.getenv("PORT", "8080"))
    workers = int(os.getenv("WORKERS", "1"))
    uvicorn.run("server:app", host="0.0.0.0", port=port, workers=workers)
