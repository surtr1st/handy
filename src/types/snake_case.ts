export type SnakeUser = {
  id: number;
  username: string;
  password: string;
};

export type AuthenticationResult = [message: string, participant_id: string];

export type SnakeIteration = {
  id: number;
  title: string;
  goals: string;
  created_by: string;
  current_point: number;
  total_point: number;
  start_date: number;
  end_date: number;
  status: boolean;
};

export type SnakeBacklog = {
  id: number;
  title: string;
  description: string;
  goals: string;
  priority: number;
  hours: number;
  points: number;
  created_date: number;
};

export type SnakeTask = {
  id: number;
  name: string;
  created_date: number;
  started_date: number;
  hours: number;
  worked_hours: number;
  mode: boolean;
  status: boolean;
  participant_id: number;
  backlog_id: number;
  pic: string;
};

export type SnakeLogwork = {
  estimated_hours: number;
  worked_hours: number;
  completed_date: number;
  task_id: number;
  pic: number;
};

export type SnakeWorklog = {
  id: number;
  completed_date: number;
  description: string;
  worked_hours: number;
  task_id: number;
};
