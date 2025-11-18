import 'package:hive_flutter/hive_flutter.dart';
import '../models/task.dart';

class StorageService {
  static const String _tasksBoxName = 'tasks';
  static Box? _tasksBox;

  static Future<void> init() async {
    // Using JSON serialization with untyped Box
    // No adapters needed
  }

  Future<Box> _getTasksBox() async {
    if (_tasksBox != null && _tasksBox!.isOpen) {
      return _tasksBox!;
    }

    _tasksBox = await Hive.openBox(_tasksBoxName);
    return _tasksBox!;
  }

  Future<void> saveTask(Task task) async {
    try {
      final box = await _getTasksBox();
      await box.put(task.id, task.toJson());
    } catch (e) {
      throw Exception('Failed to save task: $e');
    }
  }

  Future<Task?> getTask(String id) async {
    try {
      final box = await _getTasksBox();
      final value = box.get(id);

      if (value == null) return null;

      if (value is! Map) {
        throw Exception('Invalid task data format');
      }

      return Task.fromJson(Map<String, dynamic>.from(value));
    } catch (e) {
      throw Exception('Failed to get task: $e');
    }
  }

  Future<List<Task>> getAllTasks() async {
    try {
      final box = await _getTasksBox();
      final tasks = <Task>[];

      for (var value in box.values) {
        if (value is Map) {
          try {
            tasks.add(Task.fromJson(Map<String, dynamic>.from(value)));
          } catch (e) {
            // Skip invalid tasks
            continue;
          }
        }
      }

      return tasks;
    } catch (e) {
      throw Exception('Failed to get all tasks: $e');
    }
  }

  Future<void> deleteTask(String id) async {
    try {
      final box = await _getTasksBox();
      await box.delete(id);
    } catch (e) {
      throw Exception('Failed to delete task: $e');
    }
  }

  Future<void> clear() async {
    try {
      final box = await _getTasksBox();
      await box.clear();
    } catch (e) {
      throw Exception('Failed to clear tasks: $e');
    }
  }
}
