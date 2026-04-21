<script setup lang="ts">
import BackPage from '@/components/BackPage.vue';
import OnlyoneWriter from '@/components/OnlyoneWriter.vue';
import TableWriter from '@/components/TableWriter.vue';
import { toAnswerTable, type AnswerTable, type Table } from '@/utils/interfaces';
import { getTable, saveAnswer } from '@/utils/req';
import { back } from '@/utils/route';
import { onMounted, ref, type Ref } from 'vue';
import { useRoute } from 'vue-router';

const edittingTable: Ref<Table | null> = ref(null);
const answerTable: Ref<AnswerTable | null> = ref(null);

onMounted(async () => {
    const tname = useRoute().params.name as string;
    console.log(tname);
    edittingTable.value = (await getTable(tname)) as Table
    answerTable.value = toAnswerTable(edittingTable.value);
    console.log(answerTable.value);
    if (typeof edittingTable.value.name !== 'string') {
        back()
    }
})

const submitAnswer = async () => {
    if (!answerTable.value) {
        return
    }
    console.log(answerTable.value)
    let r = saveAnswer(answerTable.value)
    if ((await r).ok) {
        alert("成功上传")
    } else {
        alert("上传失败")
    }
}

</script>

<template>
    <div id="" class="fullscreen" v-if="edittingTable && answerTable">
        <div class="col">
            <div>({{ edittingTable.name }})</div>
            <div class="title">
                {{ edittingTable.title }}
            </div>
        </div>
        <TableWriter v-model="answerTable" :table="edittingTable" @submit="submitAnswer">

        </TableWriter>
        <BackPage></BackPage>
    </div>
    <div v-else class="fullscreen">
        <BackPage></BackPage>
        <div class="title">404 NOT FOUND</div>
    </div>
</template>