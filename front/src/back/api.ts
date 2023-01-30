
let api0: Api;
let uapi0: UnhandledApi;

export function api() {
    if (!api0) {
        api0 = new Api();
    }
    return api0;
}

export class Api {

    private endpoint: string = import.meta.env.VITE_ENDPOINT;

    async getNonce(addr: string) {
        let response = await this.get<{ nonce: string }>("nonce/" + addr);
        return response;
    }

    async authenticate(address: string, nonce: string, signature: string) {
        let param = new URLSearchParams();
        param.append("address", address);
        param.append("nonce", nonce);
        param.append("signature", signature.substring(2));
        let response = await this.get<{ success: boolean }>("authenticate?" + param.toString());
        return response;
    }

    private async get<T>(path: string): Promise<ApiResponse<T>> {
        let data = await fetch(this.endpoint + "/" + path);
        if (data.status == 200) {
            let jso = await data.json();
            return { success: true, data: jso.data as T };
        } else if (data.status == 500) {
            let jso = await data.json();
            return { success: false, err: jso.error as string, code: 500 };
        }
        return { success: false, code: data.status, err: data.statusText };
    }
}

export type ApiResponse<T> = {
    success: true,
    data: T
} | {
    success: false,
    code: number,
    err: string
}

export function uapi() {
    if (!uapi0) {
        uapi0 = new UnhandledApi();
    }
    return uapi0;
}

export class ApiError extends Error {
}

export class UnhandledApi {
    async getNonce(addr: string) {
        let response = await api().getNonce(addr);
        if (response.success) return response.data.nonce;
        throw new ApiError(response.err);
    }
    async authenticate(address: string, nonce: string, signature: string) {
        let response = await api().authenticate(address, nonce, signature);
        if (response.success) return response.data.success;
        throw new ApiError(response.err);
    }
}