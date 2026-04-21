import { ref } from "vue";
import type { AnswerTable, BaseInfo, Table, TableRecord } from "./interfaces";

export const apikey = ref("");

export const setApikey = (key: string) => {
    apikey.value = key
    localStorage.setItem("apikey", apikey.value);
}

export const removeApikey = () => {
    if (!confirm("确定注销吗")) {
        return
    }
    apikey.value = "";
    localStorage.removeItem("apikey")
}

export const loadApikey = () => {
    if (apikey.value === "") {
        apikey.value = localStorage.getItem("apikey") || "-";
    }
}

const request_post = async (url: string, data: any,) => {
    loadApikey()
    return (await fetch(url, {
        headers: {
            "x-apikey": apikey.value,
            "Content-Type": "application/json"
        },
        method: "POST",
        body: JSON.stringify(data)
    }))
}

const request_get = async (url: string) => {
    loadApikey()
    return (await fetch(url, {
        headers: {
            "x-apikey": apikey.value
        },
    }))
}

export const getTables = async () => {
    return (await (await request_get("/api/get_tables")).json()) as Table[]
}

export const getTable = async (name: string) => {
    return await (await request_get("/api/get_table/" + name)).json() as Table
}

export const getBaseInfo = async () => {
    return await (await request_get("/api/base")).json() as BaseInfo
}

export const saveTable = async (table: Table) => {
    return (await request_post('/api/save_table', table));
}

export const checkAuth = async () => {
    return (await fetch("/api/check_auth/" + apikey.value))
}

export const removeTable = async (tname: string) => {
    return (await request_get('/api/del_table/' + tname))
}

export const saveAnswer = async (answer: AnswerTable) => {
    return (await request_post('/api/save_record', answer));
}

export const getRecords = async (tname: string) => {
    return await ((await request_get("/api/get_records/" + tname)).json()) as TableRecord[]
}