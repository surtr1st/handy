export type ParticipantStatistic = {
  current: number;
  attended: number;
  finished: number;
  created: number;
};

export type IterationStatistic = {
  backlog: number;
  backlog_done: number;
  backlog_partially_done: number;
  backlog_undone: number;
  sprint_velocity: number;
};
