<template>
    <div class="col" v-if="model">
        <span>多选题选项:</span>
        <div v-for="(opt, i) in modelValue" :key="i" class="row">
            <input type="radio" disabled style="width: 30px;" />
            <input v-model="model[i]" class="" />
            <span @click=" remove(i)" class="button" style="width: 80px;">删除</span>
        </div>
        <div @click="add" class="button">+
            添加选项
        </div>
    </div>
</template>

<script setup lang="ts">

const model = defineModel<string[]>()

const add = () => {
    model.value?.push('')
}

const remove = (index: number) => {
    model.value?.splice(index, 1)
}
</script>