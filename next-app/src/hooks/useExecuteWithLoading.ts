import { useState } from 'react';

const useExecuteWithLoading = () => {
  const [isLoading, setIsLoading] = useState(false);

  const executeWithLoading = async <T>(callback: () => Promise<T>) => {
    setIsLoading(true);

    try {
      return await callback();
    } finally {
      setIsLoading(false);
    }
  };

  return {
    isLoading,
    executeWithLoading,
  };
};

export default useExecuteWithLoading;
