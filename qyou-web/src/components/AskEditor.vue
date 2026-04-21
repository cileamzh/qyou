<template>
    <div class="col" v-if="model !== undefined">
        <div class="col">
            简答题：用户将看到一个文本输入框。
            <input type="text" v-model="model" placeholder="你可以给用户一些提示">
        </div>
    </div>
</template>

<script setup lang="ts">
// 简答题通常不需要额外的选项配置
const model = defineModel<string>()
</script>