type User = { id: string; name: string };

// Imaginary database
const users: User[] = [{ id: "1", name: "Carlo" }];

export const db = {
  user: {
    findMany: async () => users,
  },
};
