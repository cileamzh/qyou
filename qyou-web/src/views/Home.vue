<template>
    <div id="home" class="fullscreen">
        <div id="web-title" class="title">
            欢迎来到{{ company }}问卷平台
        </div>
        <div id="route-box" class="card">
            <div class="button" @click="editTable">
                建表</div>
            <div class="button" @click="analyzeTable">
                分析</div>
            <div class="button" @click="route('/tables/write')">
                填表</div>
        </div>
        <div class="hover-button" @click="removeApikey">
            注销
        </div>
    </div>
</template>
<script setup lang="ts">
import { apikey, checkAuth, getBaseInfo, removeApikey } from '@/utils/req';
import { route } from '@/utils/route';
import { onMounted, ref } from 'vue';
const company = ref('');
const title = ref('');
onMounted(async () => {
    let bi = (await getBaseInfo())
    company.value = bi.company
    title.value = bi.web_title
})

const editTable = async () => {
    let r = await checkAuth();
    if (r.ok) {
        route('/tables/edit')
    } else {
        route("/login")
    }
}

const analyzeTable = async () => {
    let r = await checkAuth();
    if (r.ok) {
        route('/tables/analyze')
    } else {
        route("/login")
    }

}
</script>

<style scoped>
#home {
    align-items: center;
    justify-content: center;
    gap: 40px;
}

#route-box {
    max-width: 400px;
}
</style>