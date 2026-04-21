<template>
    <div class="fullscreen" id="tables-main">
        <BackPage></BackPage>

        <div id="notice" class="title" v-if="target == 'edit'">
            请选择要编辑的表格
        </div>
        <div v-else class="title">
            选择你要填的表格
        </div>
        <div class="card" id="tables-routes">
            <div v-for="table in tables" class="button" @click="openTable(table.name, target)">
                {{ table.title }}
            </div>
            <div class="button" v-if="target == 'edit'" @click="createTable">新建表单</div>
            <div v-if="(target == 'analyze' || target == 'write') && tables.length < 1">
                暂无表单
            </div>
        </div>
    </div>
</template>


<script setup lang="ts">
import BackPage from "@/components/BackPage.vue";
import type { Table } from "@/utils/interfaces";
import { getTables, saveTable } from "@/utils/req";
import { openTable } from "@/utils/route";
import { onMounted, ref, type Ref } from "vue";
import { useRoute } from "vue-router";

const target: Ref<'analyze' | 'write' | 'edit'> = ref('write')
const tables: Ref<Table[]> = ref([]);
onMounted(async () => {
    const params = useRoute().params.target;
    console.log(params)
    if (params === 'edit') {
        target.value = 'edit'
    }
    if (params === 'analyze') {
        target.value = 'analyze'
    }
    console.log(target.value)
    tables.value = await getTables();
})


const createTable = async () => {
    let name = Date.now() + "";
    let title = prompt("新表的标题", "表" + name) || null;
    if (title == null) {
        return
    }
    let table: Table = { name, title, questions: [], enable: false, know_who: false };
    await saveTable(table);
    openTable(name, 'edit');
}
</script>

<style scoped>
#tables-main {
    justify-content: center;
    align-items: center;
}

#tables-routes {
    max-width: 400px;
}
</style>