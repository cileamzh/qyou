<template>
    <div class="col" style="width: 100%;">
        <div>{{ notification }}</div>
        <textarea v-model="model.answer" placeholder="请在此输入您的回答..." rows="4" class="bg-element"
            style="width: 100%; padding: 10px; border-radius: var(--base-radius); border: none; outline: none; background: var(--button-bg);"></textarea>
    </div>
</template>

<script setup lang="ts">
const { notification } = defineProps<{ notification: string }>()
const model = defineModel<{ answer: string }>({ required: true });
</script>