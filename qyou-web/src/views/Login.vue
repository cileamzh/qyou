<template>
    <div class="col">
        <div class="title">请输入apikey</div>
        <input type="text" v-model="key" @input="(v) => {
        }" @keydown.enter="login">
        <div class="button" @click="login">提交</div>
        <BackPage></BackPage>
    </div>
</template>

<script setup lang="ts">
import BackPage from '@/components/BackPage.vue';
import router from '@/router';
import { apikey, checkAuth, setApikey } from '@/utils/req';
import { ref } from 'vue';
const key = ref('')
const login = async () => {
    setApikey(key.value)
    console.log(apikey)
    let r = await checkAuth()
    if (r.ok) {
        router.back();
    } else {
        alert("failed")
    }
}
</script>