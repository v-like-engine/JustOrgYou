import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:equatable/equatable.dart';
import '../models/task.dart';
import '../repositories/task_repository.dart';

// Events
abstract class TaskEvent extends Equatable {
  const TaskEvent();

  @override
  List<Object?> get props => [];
}

class LoadTasks extends TaskEvent {}

class AddTask extends TaskEvent {
  final String title;
  final String? body;
  final String keyword;
  final List<String> tags;

  const AddTask({
    required this.title,
    this.body,
    this.keyword = 'INBOX',
    this.tags = const [],
  });

  @override
  List<Object?> get props => [title, body, keyword, tags];
}

class UpdateTask extends TaskEvent {
  final Task task;

  const UpdateTask(this.task);

  @override
  List<Object> get props => [task];
}

class DeleteTask extends TaskEvent {
  final String taskId;

  const DeleteTask(this.taskId);

  @override
  List<Object> get props => [taskId];
}

class ToggleTaskDone extends TaskEvent {
  final String taskId;

  const ToggleTaskDone(this.taskId);

  @override
  List<Object> get props => [taskId];
}

class FilterTasks extends TaskEvent {
  final String? keyword;
  final String? tag;

  const FilterTasks({this.keyword, this.tag});

  @override
  List<Object?> get props => [keyword, tag];
}

// States
abstract class TaskState extends Equatable {
  const TaskState();

  @override
  List<Object?> get props => [];
}

class TaskInitial extends TaskState {}

class TaskLoading extends TaskState {}

class TaskLoaded extends TaskState {
  final List<Task> tasks;
  final String? filterKeyword;
  final String? filterTag;

  const TaskLoaded({
    required this.tasks,
    this.filterKeyword,
    this.filterTag,
  });

  List<Task> get filteredTasks {
    var filtered = tasks;

    if (filterKeyword != null) {
      filtered = filtered.where((t) => t.keyword == filterKeyword).toList();
    }

    if (filterTag != null) {
      filtered = filtered.where((t) => t.tags.contains(filterTag)).toList();
    }

    return filtered;
  }

  List<Task> get inboxTasks =>
      tasks.where((t) => t.keyword == 'INBOX').toList();

  List<Task> get todoTasks => tasks.where((t) => t.keyword == 'TODO').toList();

  List<Task> get doneTasks => tasks.where((t) => t.keyword == 'DONE').toList();

  List<Task> get waitingTasks =>
      tasks.where((t) => t.keyword == 'WAITING').toList();

  List<Task> get somedayTasks =>
      tasks.where((t) => t.keyword == 'SOMEDAY').toList();

  List<Task> get overdueTasks => tasks.where((t) => t.isOverdue).toList();

  @override
  List<Object?> get props => [tasks, filterKeyword, filterTag];
}

class TaskError extends TaskState {
  final String message;

  const TaskError(this.message);

  @override
  List<Object> get props => [message];
}

// BLoC
class TaskBloc extends Bloc<TaskEvent, TaskState> {
  final TaskRepository repository;

  TaskBloc({required this.repository}) : super(TaskInitial()) {
    on<LoadTasks>(_onLoadTasks);
    on<AddTask>(_onAddTask);
    on<UpdateTask>(_onUpdateTask);
    on<DeleteTask>(_onDeleteTask);
    on<ToggleTaskDone>(_onToggleTaskDone);
    on<FilterTasks>(_onFilterTasks);
  }

  Future<void> _onLoadTasks(LoadTasks event, Emitter<TaskState> emit) async {
    emit(TaskLoading());
    try {
      final tasks = await repository.getTasks();
      emit(TaskLoaded(tasks: tasks));
    } catch (e) {
      emit(TaskError(e.toString()));
    }
  }

  Future<void> _onAddTask(AddTask event, Emitter<TaskState> emit) async {
    try {
      await repository.addTask(
        title: event.title,
        body: event.body,
        keyword: event.keyword,
        tags: event.tags,
      );

      final tasks = await repository.getTasks();
      emit(TaskLoaded(tasks: tasks));
    } catch (e) {
      emit(TaskError(e.toString()));
    }
  }

  Future<void> _onUpdateTask(
      UpdateTask event, Emitter<TaskState> emit) async {
    try {
      await repository.updateTask(event.task);
      final tasks = await repository.getTasks();
      emit(TaskLoaded(tasks: tasks));
    } catch (e) {
      emit(TaskError(e.toString()));
    }
  }

  Future<void> _onDeleteTask(
      DeleteTask event, Emitter<TaskState> emit) async {
    try {
      await repository.deleteTask(event.taskId);
      final tasks = await repository.getTasks();
      emit(TaskLoaded(tasks: tasks));
    } catch (e) {
      emit(TaskError(e.toString()));
    }
  }

  Future<void> _onToggleTaskDone(
      ToggleTaskDone event, Emitter<TaskState> emit) async {
    if (state is! TaskLoaded) return;

    try {
      final currentState = state as TaskLoaded;
      final task = currentState.tasks.firstWhere((t) => t.id == event.taskId);

      final updatedTask = task.copyWith(
        keyword: task.isDone ? 'TODO' : 'DONE',
      );

      await repository.updateTask(updatedTask);
      final tasks = await repository.getTasks();
      emit(TaskLoaded(tasks: tasks));
    } catch (e) {
      emit(TaskError(e.toString()));
    }
  }

  Future<void> _onFilterTasks(
      FilterTasks event, Emitter<TaskState> emit) async {
    if (state is! TaskLoaded) return;

    final currentState = state as TaskLoaded;
    emit(TaskLoaded(
      tasks: currentState.tasks,
      filterKeyword: event.keyword,
      filterTag: event.tag,
    ));
  }
}
