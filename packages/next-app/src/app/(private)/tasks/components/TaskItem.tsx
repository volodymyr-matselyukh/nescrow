import { TaskStatus } from '@/enums/TaskStatus';
import { Task } from '@/types/Task';
import { getCurrencyString } from '@/utils/money';
import dayjs from 'dayjs';
import relativeTime from 'dayjs/plugin/relativeTime';
import { FC } from 'react';
dayjs.extend(relativeTime);

interface Props {
  task: Task;
}

const TaskItem: FC<Props> = ({
  task: { name, description, createdAt, reward, status },
}) => {
  return (
    <div className="flex cursor-pointer gap-2 border-2 border-dashed border-black p-2 hover:border-solid rounded-lg">
      <div>
        <div className="bg-primary p-2 rounded-lg text-white text-center">{name}</div>
        <div className="mt-5 bg-gray-200 p-5 text-sm rounded-lg">{description}</div>
      </div>

      <div className="flex min-w-44 flex-col text-sm font-medium">
        <div className="flex gap-2">
          <span>Status: </span>
          <span>{TaskStatus[status]}</span>
        </div>
        <div className="flex gap-2">
          <span>Reward: </span>
          <span className="text-green-600">{getCurrencyString(reward)}</span>
        </div>
        <div className="grow-1 flex basis-4/5 items-end gap-2 justify-self-end">
          <div className="flex gap-2">
            <span>Created&nbsp;at:</span>
            <div className="flex flex-col">
              <span>{dayjs(createdAt.toString()).format('D MMM YYYY')}</span>
              <span className='text-xs font-mono'>{dayjs(createdAt.toString()).fromNow()}</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default TaskItem;
