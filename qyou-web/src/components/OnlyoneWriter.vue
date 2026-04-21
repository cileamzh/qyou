<template>
    <div class="col">
        <div class="row">单选 </div>
        <div v-for="(opt, idx) in model.options" :key="idx" class="row button" @click="model.answer = idx"
            :class="{ 'select': idx === model.answer }">
            <div class="row">
                <input type="checkbox" :checked="model.answer == idx" />
                <span>{{ opt }}</span>
            </div>
        </div>

        <div v-if="model.answer != 255">已选择{{ model.answer }},{{ model.options[model.answer] }}</div>
    </div>
</template>

<script setup lang="ts">
// 使用命名 model 代替 props
const model = defineModel<{ options: string[], answer: number }>({ required: true });
</script>

<style scoped></style>