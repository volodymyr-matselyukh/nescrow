import { TaskStatus } from '@/enums/TaskStatus';

export interface Task {
  name: string;
  description: string;
  createdAt: Date;
  reward: number;
  status: TaskStatus;
}
