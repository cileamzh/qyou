<template>
    <div class="col">
        <div class="row">多选 </div>
        <div v-for="(opt, idx) in model.options" :key="idx" class="row button"
            :class="{ 'select': model.answer.includes(idx) }" @click="toggleAnswer(idx)"
            style="justify-content: flex-start; width: 100%;">
            <input type="checkbox" :checked="model.answer.includes(idx)" />
            <span>{{ opt }}</span>
        </div>
    </div>
</template>

<script setup lang="ts">
// 注意：遵循你接口中的拼写 'asnwer'
const model = defineModel<{ options: string[], answer: number[] }>({ required: true });

const toggleAnswer = (idx: number) => {
    const arr = [...model.value.answer];
    const index = arr.indexOf(idx);
    if (index > -1) {
        arr.splice(index, 1);
    } else {
        arr.push(idx);
    }
    model.value.answer = arr;
};
</script>

<style scoped></style>