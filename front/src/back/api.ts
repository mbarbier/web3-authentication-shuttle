import { fromUtf8, toUtf8 } from "./utils";


let api0: Api;

export function api() {
    if (!api0) {
        api0 = new Api();
    }
    return api0;
}

export class Api {

    private endpoint: string = import.meta.env.VITE_ENDPOINT;

    async getNonce(addr: string) {
        let data = await this.get("nonce/" + addr);
        console.log(data);
        return data.nonce as string;
    }

    async authenticate(address: string, nonce: string, signature: string) {
        console.log("address: " + address)
        let param = new URLSearchParams();
        param.append("address", address);
        param.append("nonce", nonce);
        param.append("signature", signature.substring(2));
        let data = await this.get("authenticate?" + param.toString());
        console.log(data);
        return data;
    }

    private async get(path: string) {
        let data = await fetch(this.endpoint + "/" + path);
        let jso = await data.json();
        return jso.data;
    }
}