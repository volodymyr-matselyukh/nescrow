import useUserStore from '@/store/userStore';
import { createClient } from '@/utils/supabase/browserClient';
import { useEffect, useRef, useState } from 'react';

export const useUser = () => {
  const getUserPromise = useRef<Promise<void>>();
  const { user, setUser } = useUserStore();
  const [error, setError] = useState<string | null>(null);
  const supabase = createClient();

  useEffect(() => {
    if (!user && !getUserPromise.current) {
      getUserPromise.current = supabase.auth
        .getUser()
        .then((userObject) => {
          setUser(userObject.data.user);
        })
        .catch((error) => {
          setError(error);
          console.log("Couldn't get user", error);
        });
    }
  }, []);

  return { user, error };
};
