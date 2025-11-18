import '../models/task.dart';
import '../services/storage_service.dart';
import 'package:uuid/uuid.dart';

class TaskRepository {
  final StorageService _storage = StorageService();
  final _uuid = const Uuid();

  Future<List<Task>> getTasks() async {
    return await _storage.getAllTasks();
  }

  Future<Task?> getTask(String id) async {
    return await _storage.getTask(id);
  }

  Future<void> addTask({
    required String title,
    String? body,
    String keyword = 'INBOX',
    List<String> tags = const [],
    DateTime? scheduled,
    DateTime? deadline,
    int? priority,
  }) async {
    final task = Task(
      id: _uuid.v4(),
      title: title,
      body: body,
      keyword: keyword,
      tags: tags,
      scheduled: scheduled,
      deadline: deadline,
      priority: priority,
    );

    await _storage.saveTask(task);
  }

  Future<void> updateTask(Task task) async {
    await _storage.saveTask(task);
  }

  Future<void> deleteTask(String id) async {
    await _storage.deleteTask(id);
  }

  Future<List<Task>> getTasksByKeyword(String keyword) async {
    final tasks = await getTasks();
    return tasks.where((t) => t.keyword == keyword).toList();
  }

  Future<List<Task>> getTasksByTag(String tag) async {
    final tasks = await getTasks();
    return tasks.where((t) => t.tags.contains(tag)).toList();
  }

  Future<List<Task>> getOverdueTasks() async {
    final tasks = await getTasks();
    return tasks.where((t) => t.isOverdue).toList();
  }

  Future<void> importFromOrgFile(String content) async {
    // TODO: Parse org mode file and import tasks
    // This would use the Rust library via FFI
  }

  Future<String> exportToOrgFile() async {
    // TODO: Export tasks to org mode format
    // This would use the Rust library via FFI
    return '';
  }
}
