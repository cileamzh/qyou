<script setup lang="ts">
import AnalyzeRecord from '@/components/AnalyzeRecord.vue';
import BackPage from '@/components/BackPage.vue';
import Debuger from '@/components/debug/Debuger.vue';
import { showDebug } from '@/utils/global';
import type { Table, TableRecord } from '@/utils/interfaces';
import { getRecords, getTable } from '@/utils/req';

import { onMounted, ref, type Ref } from 'vue';
import { useRoute } from 'vue-router';

const tname = ref()
const records: Ref<TableRecord[]> = ref([]);
const table = ref()

onMounted(async () => {
    tname.value = useRoute().params.name;
    table.value = await getTable(tname.value);
    records.value = await getRecords(tname.value);
})
</script>

<template>
    <div class="col">
        <Debuger v-model="showDebug" :data="{ tname, records }"></Debuger>
        <AnalyzeRecord :records="records" :table="table" v-if="table && records"></AnalyzeRecord>
        <BackPage></BackPage>
    </div>
</template>