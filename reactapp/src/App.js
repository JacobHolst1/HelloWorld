import './App.css';
import TodoList from './TodoList';
import React, { useState, useRef, useEffect} from 'react'
import { v4 as uuidv4 } from 'uuid'

const LOCAL_STORAGE_KEY = 'todoApp.todos'

function App() {
  const [todos, setTodos] = useState([{id: uuidv4(), name: "Sample Todo", complete: false}]);
  const todoNameRef = useRef()

  useEffect(() => {
    const storedTodos = JSON.parse(localStorage.getItem(LOCAL_STORAGE_KEY))
    if (storedTodos) setTodos(storedTodos)
  }, [])

  useEffect(() => {
    localStorage.setItem(LOCAL_STORAGE_KEY, JSON.stringify(todos))
  }, [todos])
  
  //Toggles the checkbox for a to do item
  function toggleTodo(id) {
    const newTodos = [...todos]
    const todo = newTodos.find(todo => todo.id === id)
    todo.complete = !todo.complete
    setTodos(newTodos)
  }

  //Adds a to do item to the list
  function handleAddTodo(e) {
    const name = todoNameRef.current.value

    //if there is nothing in the box, then do nothing
    if (name === '') return
    
    setTodos(prevTodos => {
      return [...prevTodos, {id:uuidv4(), name: name, complete:false}]
    })
    todoNameRef.current.value = null
  }

  //Clears all the items marked as complete
  function handleClearTodos() {
    const newTodos = todos.filter(todo => !todos.complete)
    setTodos(newTodos);
  }

  return (
    <>
      <TodoList todos={todos} toggleTodo={toggleTodo} />
      <input ref={todoNameRef} type="text" />
      <button onClick={handleAddTodo}>Add Todo</button>
      <button onClick={handleClearTodos}>Clear Complete</button>
      <div>{todos.filter(todo => !todo.complete).length} left to do</div>
    </>
  );
}

export default App;
