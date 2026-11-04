//! 文本与字节的转换：与后端 base64 契约对应。

export function encodeUtf8Base64(text: string): string {
  const bytes = new TextEncoder().encode(text);
  let binary = "";
  for (const byte of bytes) {
    binary += String.fromCharCode(byte);
  }
  return btoa(binary);
}

export function decodeUtf8Base64(dataB64: string): string {
  const binary = atob(dataB64);
  const bytes = new Uint8Array(binary.length);
  for (let index = 0; index < binary.length; index += 1) {
    bytes[index] = binary.charCodeAt(index);
  }
  return new TextDecoder().decode(bytes);
}

export function hexPreview(dataB64: string, limit = 512): string {
  const binary = atob(dataB64);
  const bytes = Array.from(binary.slice(0, limit), (char) => char.charCodeAt(0));
  const lines: string[] = [];
  for (let offset = 0; offset < bytes.length; offset += 16) {
    const chunk = bytes.slice(offset, offset + 16);
    const hex = chunk.map((byte) => byte.toString(16).padStart(2, "0")).join(" ");
    const ascii = chunk
      .map((byte) => (byte >= 32 && byte < 127 ? String.fromCharCode(byte) : "."))
      .join("");
    lines.push(`${offset.toString(16).padStart(8, "0")}  ${hex.padEnd(47)}  ${ascii}`);
  }
  return lines.join("\n");
}
