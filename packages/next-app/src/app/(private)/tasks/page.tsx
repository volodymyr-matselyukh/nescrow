import { TaskStatus } from '@/enums/TaskStatus';
import { Task } from '@/types/Task';
import TasksContainer from './components/TasksContainer';

const tasksList: Task[] = [
  {
    name: 'Mystery box refactoring',
    description:
      'Refactor mystery box so, there is no need to deploy contracts to new accounts. This will improve cost efficiency. Front-end changes and contract changes are required.',
    reward: 500,
    createdAt: new Date('7/7/2024 11:30:12'),
    status: TaskStatus.CREATED,
  },
];

const ownerTasksList: Task[] = [
  {
    name: 'Landing page creation',
    description:
      'Create a landing page for Near community to learn about near infrastructure and activities. It should be SEO optimized and written in React.JS',
    reward: 500,
    createdAt: new Date('7/7/2024 11:30:12'),
    status: TaskStatus.CREATED,
  },
];

const Page = () => {
  return <TasksContainer myFrellancerTasks={tasksList} myOwnerTasks={ownerTasksList} />;
};

export default Page;
