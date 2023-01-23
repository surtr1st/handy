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
  hours: number;
  actual_hours: number;
  progress: string;
  pic: string;
};

export type SnakeLogwork = {
  pic: string;
  estimated_hours: number;
  worked_hours: number;
  completed_date: number;
};
