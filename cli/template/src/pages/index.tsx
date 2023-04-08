import { Button } from "@/components/ui/button";
import { NextPage } from "next";
import { Input } from "@/components/ui/input";
import { useRustCommand, useRustQuery } from "@/hooks/use-rust";
import { useState } from "react";
import { Trash2, Circle, CheckCircle2 } from "lucide-react";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { ScrollArea } from "@/components/ui/scroll-area";

const Home: NextPage = () => {
  const { data, isLoading, refetch } = useRustQuery("get_todo_items");
  const { mutate: addTask, error, isError } = useRustCommand("add_todo_item");
  const { mutate: deleteTask } = useRustCommand("remove_todo_item");
  const { mutate: toggleTask } = useRustCommand("toggle_todo_item");

  const [newTodoName, setNewTodoName] = useState("");

  return (
    <main className="w-full h-screen flex items-center justify-center flex-col gap-12">
      <h1 className="text-4xl font-black">Welcome to create-taux-app!</h1>
      <h2 className="text-3xl">ToDo app demo</h2>
      <div className="flex w-full max-w-sm items-center space-x-2">
        <Input
          placeholder="New task..."
          onChange={(e) => setNewTodoName(e.target.value)}
        />
        <Button
          type="submit"
          onClick={async () => {
            try {
              await addTask({ content: newTodoName });
              await refetch();
            } catch (e) {}
          }}
        >
          Create
        </Button>
      </div>
      {isError && <span className="-mt-8 text-red-600">Error: {error}</span>}

      <div className="w-full px-20">
        {isLoading ? (
          <div>Loading...</div>
        ) : (
          <ScrollArea className="h-[150px]">
            <div className="flex items-center justify-center flex-col w-full gap-2">
              {data?.map((todo) => (
                <div
                  className="border border-gray-200 shadow-md hover:shadow-lg transition-shadow cursor-pointer p-4 w-full rounded-md flex items-center justify-between"
                  key={todo.id}
                >
                  <div
                    className="flex items-center gap-3"
                    onClick={async () => {
                      try {
                        await toggleTask({ id: todo.id });
                        await refetch();
                      } catch (e) {}
                    }}
                  >
                    {todo.completed ? (
                      <CheckCircle2 className="h-5 w-5 text-green-500" />
                    ) : (
                      <Circle className="h-5 w-5 text-gray-500" />
                    )}
                    <span className="text-xl">{todo.content}</span>
                  </div>
                  <div>
                    <TooltipProvider>
                      <Tooltip>
                        <TooltipTrigger asChild>
                          <Button
                            className="h-12 w-12"
                            variant={"outline"}
                            onClick={async () => {
                              try {
                                await deleteTask({ id: todo.id });
                                await refetch();
                              } catch (e) {}
                            }}
                          >
                            <Trash2 className="h-5 w-5" />
                          </Button>
                        </TooltipTrigger>
                        <TooltipContent>Delete task</TooltipContent>
                      </Tooltip>
                    </TooltipProvider>
                  </div>
                </div>
              ))}
            </div>
          </ScrollArea>
        )}
      </div>
    </main>
  );
};

export default Home;
