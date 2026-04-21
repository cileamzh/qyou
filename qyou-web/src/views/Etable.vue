<script setup lang="ts">
import BackPage from '@/components/BackPage.vue';
import TableEditor from '@/components/TableEditor.vue';
import type { Table } from '@/utils/interfaces';
import { getTable, removeTable, saveTable } from '@/utils/req';
import { back } from '@/utils/route';
import { onMounted, ref, watch, type Ref } from 'vue';
import { useRoute } from 'vue-router';

const edittingTable: Ref<Table | null> = ref(null);

watch(edittingTable, () => {
    console.log(edittingTable.value)
}, { deep: true })

onMounted(async () => {
    const tname = useRoute().params.name as string;
    console.log(tname);
    let r = (await getTable(tname));
    if (typeof r.name === 'string') {
        edittingTable.value = r;
    }
})

const onSave = async () => {
    if (edittingTable.value) {
        console.log(edittingTable.value)
        let r = await saveTable(edittingTable.value);
        if (r.ok) {
            alert("保存成功")
        } else {
            alert("保存失败")
        }
    }
}

const onRemove = async () => {
    if (edittingTable.value) {
        let r = await removeTable(edittingTable.value.name);
        if (r.ok) {
            alert("成功删除")
            back()
        } else {
            alert("删除失败")
        }
    }
}
</script>

<template>
    <div id="" class="fullscreen" v-if="edittingTable">
        <TableEditor v-model="edittingTable" @save="onSave" @remove="onRemove"></TableEditor>
        <BackPage></BackPage>

    </div>
    <div v-else class="fullscreen">
        <BackPage></BackPage>

        <div class="title">404 NOT FOUND</div>
    </div>
</template>