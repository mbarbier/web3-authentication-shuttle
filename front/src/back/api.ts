

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

    private async get(path: string) {
        let data = await fetch(this.endpoint + "/" + path);
        let jso = await data.json();
        return jso.data;
    }
}