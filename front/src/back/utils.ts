import * as utf8 from "utf8";

export function toUtf8(hex: string) {
    console.log(hex);
    // Find termination
    let str = "";
    // skip 0x
    for (let i = 2; i < hex.length; i += 2) {
        let code = parseInt(hex.substring(i, i + 2), 16);
        console.log(hex.substring(i, i + 2) + " => " + code + " => " + String.fromCharCode(code));
        str += String.fromCharCode(code);
    }
    return utf8.decode(str);
    //return str;
};

export function fromUtf8(str: string) {
    str = utf8.encode(str);
    let hex = "";
    for (let i = 0; i < str.length; i++) {
        var n = str.charCodeAt(i).toString(16);
        hex += n.length < 2 ? '0' + n : n;
    }
    return "0x" + hex;
};
