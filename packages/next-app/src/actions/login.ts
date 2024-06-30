'use server';

import { createClient } from '@/utils/supabase/serverClient';
import { headers } from 'next/headers';
import { redirect } from 'next/navigation';

export const signIn = async (email: string, password: string) => {
  const supabase = createClient();

  const { error } = await supabase.auth.signInWithPassword({
    email,
    password,
  });

  if (error) {
    return redirect('/login?message=Could not authenticate user');
  }

  return redirect('/home');
};

export const signUp = async (email: string, password: string) => {
  const origin = headers().get('origin');
  const supabase = createClient();

  const { error } = await supabase.auth.signUp({
    email,
    password,
    options: {
      emailRedirectTo: `${origin}/auth/callback`,
    },
  });

  if (error) {
    console.log('Error during sign up', error);
    return redirect('/login?message=Could not authenticate user');
  }

  return redirect('/login?message=Check email to continue sign in process');
};
