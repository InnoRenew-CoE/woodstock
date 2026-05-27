import { PUBLIC_API_BASE_URL } from "$env/static/public";

export class AudioRecorder {
    private mediaRecorder: MediaRecorder | null = null;
    private socket: WebSocket | null = null;
    private stream: MediaStream | null = null;

    async start(onTranscript: (text: string) => void): Promise<void> {
        this.stream = await navigator.mediaDevices.getUserMedia({ audio: true });

        this.socket = new WebSocket(`ws://${PUBLIC_API_BASE_URL}/audio`);
        this.socket.binaryType = 'arraybuffer';

        this.socket.onmessage = (e: MessageEvent) => {
            onTranscript(e.data as string);
        };

        this.socket.onerror = (e) => {
            console.error('WebSocket error:', e);
        };

        await this.waitForSocket();

        this.mediaRecorder = new MediaRecorder(this.stream, {
            mimeType: 'audio/webm;codecs=opus',
        });

        this.mediaRecorder.ondataavailable = (e: BlobEvent) => {
            if (e.data.size > 0 && this.socket?.readyState === WebSocket.OPEN) {
                e.data.arrayBuffer().then((buf) => this.socket?.send(buf));
            }
        };

        this.mediaRecorder.start(250);
    }

    stop(): void {
        this.mediaRecorder?.stop();
        this.stream?.getTracks().forEach((t) => t.stop());
        this.socket?.close();

        this.mediaRecorder = null;
        this.stream = null;
        this.socket = null;
    }

    get recording(): boolean {
        return this.mediaRecorder?.state === 'recording';
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

