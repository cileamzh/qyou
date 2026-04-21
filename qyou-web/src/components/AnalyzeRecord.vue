<template>
    <div class="fullscreen col">
        <div class="row"><span>总调查数量：{{ records.length }}</span> <span>标题：{{ table.title
                }}</span><span>表名：{{ table.name }}</span></div>
        <div class="card" v-for="(question, qidx) in table.questions">
            <div class="row">
                <span>{{ question.question }}</span>
                <span>合计：{{ getTotal(qidx) }}</span>
            </div>
            <div v-if="'Onlyone' in question.choice" class="col">
                <div v-for="(opt, oidx) in question.choice.Onlyone.options" class="row">
                    <span>{{ opt }}</span> <span>{{ getCounts(qidx, oidx) }}</span>
                </div>
            </div>
            <div v-if="'Multiple' in question.choice" class="col">
                <div v-for="(opt, oidx) in question.choice.Multiple.options" class="row">
                    <span>{{ opt }}</span> <span>{{ getCounts(qidx, oidx) }}</span>
                </div>
            </div>
            <div v-if="'Ask' in question.choice" class="col">
                <div></div>
                <div class="button">随机抽取</div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import type { Table, TableRecord } from '@/utils/interfaces';

const { table, records } = defineProps<{ table: Table, records: TableRecord[] }>()

const getTotal = (qidx: number) => {
    let count = 0;
    records.forEach(rec => {
        let qs = rec.answers[qidx];
        if (!qs) {
            return
        }
        let qc = qs.choice;
        if ('Onlyone' in qc) {
            if (qc.Onlyone.answer !== 255) {
                count++;
            }
        }
        if ('Multiple' in qc) {
            if (qc.Multiple.answer.length > 0) {
                count++
            }
        }
        if ('Ask' in qc) {
            if (qc.Ask.answer.length > 0) {
                count++
            }
        }
    })
    return count
}

const getCounts = (qidx: number, oidx: number) => {
    let count = 0;
    records.forEach(rec => {
        let qs = rec.answers[qidx];
        if (!qs) {
            return
        }
        let qc = qs.choice;
        if ('Onlyone' in qc) {
            if (qc.Onlyone.answer === oidx) {
                count++;
            }
        }
        if ('Multiple' in qc) {
            if (qc.Multiple.answer.includes(oidx)) {
                count++
            }
        }
    })
    return count
}

const randomAsk=()=>{

}

</script>