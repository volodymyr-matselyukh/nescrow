import { createClient } from "@/utils/supabase/browserClient";
import { User } from "@supabase/supabase-js";
import { useEffect, useState } from "react";

export const useUser = () => {
  const [user, setUser] = useState<User | null>();
  const [error, setError] = useState<string | null>(null);
  const supabase = createClient();

  useEffect(() => {
    supabase.auth
      .getUser()
      .then((userObject) => {
        setUser(userObject.data.user);
      })
      .catch((error) => {
        setError(error);
        console.log("Couldn't get user", error);
      });
  }, []);

  return { user, error }
}