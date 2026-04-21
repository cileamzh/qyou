import router from "@/router"


export const openTable = (name: string, method: "edit" | "write" | 'analyze') => {
    if (method === "edit") {
        route('/etable/' + name)
    }
    if (method === 'write') {
        route('/wtable/' + name)
    }
    if (method === 'analyze') {
        route('/atable/' + name)
    }
}

export const route = (path: string) => {
    router.push(path)
}

export const back = () => {
    router.back()
}