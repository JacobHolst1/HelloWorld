package com.example.todolist;

import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.widget.CheckBox;

import androidx.annotation.NonNull;
import androidx.recyclerview.widget.RecyclerView;

import java.util.List;

public class ToDoAdapter extends RecyclerView.Adapter<ToDoAdapter.ViewHolder> {
    private List<ToDoModel> todolist;
    private MainActivity activity;

    public ToDoAdapter(MainActivity activity){
        this.activity=activity;
    }

    @NonNull
    @Override
    public ToDoAdapter.ViewHolder onCreateViewHolder(@NonNull ViewGroup parent, int viewType) {
        View itemView = LayoutInflater.from(parent.getContext()).inflate(R.layout.task_layout, parent, false);
        return new RecyclerView.ViewHolder(itemView);
    }
    public void onBindViewHolder(ViewHolder holder, int position){
        ToDoModel item = todolist.get(position);
        holder.task.setText(item.getTask());
        holder.task.setChecked(toBoolean(item.getStatus()));
    }
    public int getItemCount(){
        return todolist.size();
    }
    private static boolean toBoolean(int n){
        return n!=0;
    }

    public void setTasks(List<ToDoModel> todolist){
        this.todolist = todolist;
        notifyDataSetChanged();
    }
    public static class ViewHolder extends  RecyclerView.ViewHolder{
        CheckBox task;

        ViewHolder(View view){
            super(view);
            task=view.findViewById(R.id.todoCheckBox);
        }
    }
}
