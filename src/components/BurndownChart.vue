<script setup lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import { onMounted, reactive, ref } from 'vue';
  import { colors } from '../configs/colors';
  import { useIterationRoute } from '../store';
  import {
    Chart,
    ChartData,
    ChartConfiguration,
    ChartOptions,
  } from 'chart.js/auto';

  const { iterationId } = useIterationRoute();
  const canvas = ref();
  const burndown = reactive({
    days: [] as string[],
    totalPoint: 0,
    currentPoints: [] as number[],
    idealPoints: [] as number[],
  });

  function showBurndown() {
    const speedData: ChartData = {
      labels: burndown.days,
      datasets: [
        {
          label: 'Actual',
          data: burndown.currentPoints,
          fill: false,
          borderColor: colors.primary,
          backgroundColor: colors.primary,
        },
        {
          label: 'Ideal',
          borderColor: '#6C8893',
          backgroundColor: '#6C8893',
          borderDash: [2, 2],
          fill: false,
          data: burndown.idealPoints,
        },
      ],
    };

    const chartOptions: ChartOptions = {
      scales: {
        yAxes: {
          ticks: {
            minRotation: 0,
            maxRotation: Math.round(burndown.totalPoint * 1.1),
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

  function fetchTotalPoint() {
    invoke<number>('get_total_point', { iterationId })
      .then((res) => (burndown.totalPoint = res))
      .catch((e) => console.log(e));
  }

  function fetchTotalDay() {
    invoke<Array<string>>('get_days', { iterationId })
      .then((res) => (burndown.days = res))
      .catch((e) => console.log(e));
  }

  function fetchCurrentPointOfDay() {
    invoke<Array<number>>('get_current_points', { iterationId })
      .then((res) => (burndown.currentPoints = res))
      .catch((e) => console.log(e));
  }

  function fetchIdealPoints() {
    invoke<Array<number>>('get_ideal_points', { iterationId })
      .then((res) => (burndown.idealPoints = res))
      .catch((e) => console.log(e));
  }

  onMounted(() => {
    fetchTotalPoint();
    fetchTotalDay();
    fetchCurrentPointOfDay();
    fetchIdealPoints();
    setTimeout(() => {
      showBurndown();
    }, 250);
  });
</script>

<template>
  <canvas ref="canvas" />
</template>
