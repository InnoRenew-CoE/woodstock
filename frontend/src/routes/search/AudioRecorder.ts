import { PUBLIC_API_BASE_URL } from "$env/static/public";

export class AudioRecorder {
    private stream: MediaStream | null = null;
    private audioContext: AudioContext | null = null;
    private chunks: Float32Array[] = [];

    async start(): Promise<void> {
        this.chunks = [];
        this.stream = await navigator.mediaDevices.getUserMedia({ audio: true });
        this.audioContext = new AudioContext({ sampleRate: 16000 });
        const source = this.audioContext.createMediaStreamSource(this.stream);
        await this.audioContext.audioWorklet.addModule(this.createWorkletURL());

        const worklet = new AudioWorkletNode(this.audioContext, 'pcm-processor');
        source.connect(worklet);

        worklet.port.onmessage = (e: MessageEvent<Float32Array>) => {
            this.chunks.push(new Float32Array(e.data));
        };
    }

    async stop(onTranscript: (text: string) => void): Promise<void> {
        this.audioContext?.close();
        this.stream?.getTracks().forEach((t) => t.stop());

        const pcm = this.mergeChunks();
        const wav = this.addWavHeader(this.float32ToPCM16(pcm), 16000);

        this.chunks = [];
        this.audioContext = null;
        this.stream = null;

        const formData = new FormData();
        formData.append('file', wav, 'recording.wav');
        // formData.append('model', 'whisper-1');
        // formData.append('language', 'en');

        const res = await fetch(`${PUBLIC_API_BASE_URL}/transcribe`, {
            method: 'POST',
            body: formData,
        });

        const data = await res.json();
        onTranscript(data.text);
    }

    private mergeChunks(): Float32Array {
        const totalLength = this.chunks.reduce((acc, c) => acc + c.length, 0);
        const merged = new Float32Array(totalLength);
        let offset = 0;
        for (const chunk of this.chunks) {
            merged.set(chunk, offset);
            offset += chunk.length;
        }
        return merged;
    }

    private float32ToPCM16(float32: Float32Array): ArrayBuffer {
        const buffer = new ArrayBuffer(float32.length * 2);
        const view = new DataView(buffer);
        for (let i = 0; i < float32.length; i++) {
            const s = Math.max(-1, Math.min(1, float32[i]));
            view.setInt16(i * 2, s < 0 ? s * 0x8000 : s * 0x7fff, true);
        }
        return buffer;
    }

    private addWavHeader(pcm: ArrayBuffer, sampleRate: number): Blob {
        const numChannels = 1;
        const bitsPerSample = 16;
        const byteRate = (sampleRate * numChannels * bitsPerSample) / 8;
        const blockAlign = (numChannels * bitsPerSample) / 8;
        const dataSize = pcm.byteLength;
        const buffer = new ArrayBuffer(44 + dataSize);
        const view = new DataView(buffer);

        const write = (offset: number, str: string) =>
            [...str].forEach((c, i) => view.setUint8(offset + i, c.charCodeAt(0)));

        write(0, 'RIFF');
        view.setUint32(4, 36 + dataSize, true);
        write(8, 'WAVE');
        write(12, 'fmt ');
        view.setUint32(16, 16, true);
        view.setUint16(20, 1, true);
        view.setUint16(22, numChannels, true);
        view.setUint32(24, sampleRate, true);
        view.setUint32(28, byteRate, true);
        view.setUint16(32, blockAlign, true);
        view.setUint16(34, bitsPerSample, true);
        write(36, 'data');
        view.setUint32(40, dataSize, true);
        new Uint8Array(buffer, 44).set(new Uint8Array(pcm));

        return new Blob([buffer], { type: 'audio/wav' });
    }

    get recording(): boolean {
        return !!this.audioContext && this.audioContext.state === 'running';
    }

    private createWorkletURL(): string {
        const code = `
      class PCMProcessor extends AudioWorkletProcessor {
        process(inputs) {
          const input = inputs[0][0];
          if (input) this.port.postMessage(input);
          return true;
        }
      }
      registerProcessor('pcm-processor', PCMProcessor);
    `;
        return URL.createObjectURL(new Blob([code], { type: 'application/javascript' }));
    }
}