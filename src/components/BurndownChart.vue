<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue';
import { colors } from '../configs/colors';
import {
  Chart,
  ChartData,
  ChartConfiguration,
  ChartOptions,
} from 'chart.js/auto';

const canvas = ref();

function sumArrayUpTo(arrData: Array<number>, index: number) {
  let total = 0;
  for (let i = 0; i <= index; i++) {
    if (arrData.length > i) {
      total += arrData[i];
    }
  }
  return total;
}

function calculateIdeal(
  burndownData: Array<number>,
  scopeChange: Array<number>,
  index: number,
): number {
  const totalHoursInSprint = burndownData[0];
  const idealHoursPerDay = totalHoursInSprint / 9;
  return Math.round(
    totalHoursInSprint -
      idealHoursPerDay * index++ +
      sumArrayUpTo(scopeChange, index),
  );
}

function showBurnDown(burndownData: Array<number>, scopeChange: Array<number>) {
  const speedData: ChartData = {
    labels: [
      'Day 1',
      'Day 2',
      'Day 3',
      'Day 4',
      'Day 5',
      'Day 6',
      'Day 7',
      'Day 8',
      'Day 9',
      'Day 10',
    ],
    datasets: [
      {
        label: 'Reality',
        data: burndownData,
        fill: false,
        borderColor: colors.primary,
        backgroundColor: colors.primary,
      },
      {
        label: 'Ideally',
        borderColor: '#6C8893',
        backgroundColor: '#6C8893',
        borderDash: [2, 2],
        fill: false,
        data: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9].map((item, index) =>
          calculateIdeal(burndownData, scopeChange, index),
        ),
      },
    ],
  };

  const chartOptions: ChartOptions = {
    scales: {
      yAxes: {
        ticks: {
          minRotation: 0,
          maxRotation: Math.round(burndownData[0] * 1.1),
        },
      },
    },
  };

  const chartConfig: ChartConfiguration = {
    type: 'line',
    data: speedData,
    options: chartOptions,
  };

  new Chart(canvas.value, chartConfig);
}

onMounted(() => {
  showBurnDown(
    [200, 160, 160, 140, 90, 90, 80],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
  );
  window.addEventListener('resize', () => {
    const width = window.innerWidth;
    const height = window.innerHeight;
    const cv = canvas.value as HTMLCanvasElement;
    cv.width = width;
    cv.height = height;
  });
});
onUnmounted(() => {
  window.removeEventListener('resize', (e: Event) => {});
});
</script>

<template>
  <canvas ref="canvas" />
</template>
