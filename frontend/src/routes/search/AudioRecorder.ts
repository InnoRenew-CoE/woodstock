import { PUBLIC_API_BASE_URL } from "$env/static/public";

export class AudioRecorder {
    private mediaRecorder: MediaRecorder | null = null;
    private socket: WebSocket | null = null;
    private stream: MediaStream | null = null;
    private audioContext: AudioContext | null = null;

    async start(onTranscript: (text: string) => void): Promise<void> {
        this.stream = await navigator.mediaDevices.getUserMedia({ audio: true });

        this.socket = new WebSocket(`ws://${PUBLIC_API_BASE_URL}/audio`);
        this.socket.binaryType = 'arraybuffer';

        this.socket.onopen = () => {
            // Send WhisperLive config first
            this.socket?.send(JSON.stringify({
                uid: crypto.randomUUID(),
                language: 'en',
                model: 'base',
                use_vad: true,
            }));
        };

        this.socket.onmessage = (e: MessageEvent) => {
            const data = JSON.parse(e.data);
            if (data.segments) {
                const text = data.segments.map((s: { text: string }) => s.text).join(' ');
                onTranscript(text);
            }
        };

        this.socket.onerror = (e) => console.error('WebSocket error:', e);

        await this.waitForSocket();
        await this.startPCMStream();
    }

    // WhisperLive expects 16-bit PCM at 16kHz, not webm
    private async startPCMStream(): Promise<void> {
        this.audioContext = new AudioContext({ sampleRate: 16000 });
        const source = this.audioContext.createMediaStreamSource(this.stream!);
        await this.audioContext.audioWorklet.addModule(this.createWorkletURL());

        const worklet = new AudioWorkletNode(this.audioContext, 'pcm-processor');
        source.connect(worklet);

        worklet.port.onmessage = (e: MessageEvent<Float32Array>) => {
            if (this.socket?.readyState === WebSocket.OPEN) {
                const pcm16 = this.float32ToPCM16(e.data);
                this.socket.send(pcm16);
            }
        };
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

    // Inline worklet as a blob URL to avoid needing a separate file
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

    stop(): void {
        this.mediaRecorder?.stop();
        this.audioContext?.close();
        this.stream?.getTracks().forEach((t) => t.stop());
        this.socket?.close();

        this.mediaRecorder = null;
        this.audioContext = null;
        this.stream = null;
        this.socket = null;
    }

    get recording(): boolean {
        return !!this.audioContext && this.audioContext.state === 'running';
    }

    private waitForSocket(): Promise<void> {
        return new Promise((resolve, reject) => {
            if (!this.socket) return reject('No socket');
            if (this.socket.readyState === WebSocket.OPEN) return resolve();
            this.socket.onopen = () => resolve();
            this.socket.onerror = () => reject('Socket failed to connect');
        });
    }
}
