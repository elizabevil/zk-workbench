//! 背景图片存取。

import {call} from "./client";

export function backgroundImageStore(dataB64: string, extension: string): Promise<string> {
    return call<string>("background_image_store", {dataB64, extension});
}

export function backgroundImageLoad(path: string): Promise<string> {
    return call<string>("background_image_load", {path});
}
