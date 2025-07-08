import {invoke} from "@tauri-apps/api/core";

const DEFAULT_CHUNK_SIZE = 64 * 1024;

export function readUint8Array(reader, offset, size, diskNumber) {
    return reader.readUint8Array(offset, size, diskNumber);
}

export class BlobReader {
    constructor(filePath, size) {
        this.filePath = filePath;
        this.size = size;//await invoke('read_file_size', {path: this.filePath});
    }

    async readUint8Array(offset, length) {
        const blob = await invoke('file_seek_read', {path: this.filePath, offset, length});
        return new Uint8Array(blob);
    }

    get readable() {
        const reader = this;
        const {chunkSize = DEFAULT_CHUNK_SIZE} = reader;
        const readable = new ReadableStream({
            start() {
                this.chunkOffset = 0;
            },
            async pull(controller) {
                const {offset = 0, size, diskNumberStart} = readable;
                const {chunkOffset} = this;
                controller.enqueue(await readUint8Array(reader, offset + chunkOffset, Math.min(chunkSize, size - chunkOffset), diskNumberStart));
                if (chunkOffset + chunkSize > size) {
                    controller.close();
                } else {
                    this.chunkOffset += chunkSize;
                }
            }
        });
        return readable;
    }
}

export function naturalSort(a, b) {
    return a.filename.localeCompare(b.filename, undefined, {numeric: true, sensitivity: 'base'});
}