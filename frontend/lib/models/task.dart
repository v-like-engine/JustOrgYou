import 'package:equatable/equatable.dart';

class Task extends Equatable {
  final String id;
  final String title;
  final String? body;
  final List<Task> children;
  final int? priority;
  final String keyword;
  final List<String> tags;
  final DateTime? scheduled;
  final DateTime? deadline;
  final Map<String, String> properties;

  const Task({
    required this.id,
    required this.title,
    this.body,
    this.children = const [],
    this.priority,
    this.keyword = 'TODO',
    this.tags = const [],
    this.scheduled,
    this.deadline,
    this.properties = const {},
  });

  bool get isDone => keyword == 'DONE';

  bool get isOverdue {
    if (deadline == null) return false;
    return deadline!.isBefore(DateTime.now());
  }

  Task copyWith({
    String? id,
    String? title,
    String? body,
    List<Task>? children,
    int? priority,
    String? keyword,
    List<String>? tags,
    DateTime? scheduled,
    DateTime? deadline,
    Map<String, String>? properties,
  }) {
    return Task(
      id: id ?? this.id,
      title: title ?? this.title,
      body: body ?? this.body,
      children: children ?? this.children,
      priority: priority ?? this.priority,
      keyword: keyword ?? this.keyword,
      tags: tags ?? this.tags,
      scheduled: scheduled ?? this.scheduled,
      deadline: deadline ?? this.deadline,
      properties: properties ?? this.properties,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'title': title,
      'body': body,
      'children': children.map((c) => c.toJson()).toList(),
      'priority': priority,
      'keyword': keyword,
      'tags': tags,
      'scheduled': scheduled?.toIso8601String(),
      'deadline': deadline?.toIso8601String(),
      'properties': properties,
    };
  }

  factory Task.fromJson(Map<String, dynamic> json) {
    return Task(
      id: json['id'] as String,
      title: json['title'] as String,
      body: json['body'] as String?,
      children: (json['children'] as List<dynamic>?)
              ?.map((c) => Task.fromJson(c as Map<String, dynamic>))
              .toList() ??
          [],
      priority: json['priority'] as int?,
      keyword: json['keyword'] as String? ?? 'TODO',
      tags: (json['tags'] as List<dynamic>?)
              ?.map((t) => t as String)
              .toList() ??
          [],
      scheduled: json['scheduled'] != null
          ? DateTime.parse(json['scheduled'] as String)
          : null,
      deadline: json['deadline'] != null
          ? DateTime.parse(json['deadline'] as String)
          : null,
      properties: (json['properties'] as Map<String, dynamic>?)
              ?.map((k, v) => MapEntry(k, v as String)) ??
          {},
    );
  }

  @override
  List<Object?> get props => [
        id,
        title,
        body,
        children,
        priority,
        keyword,
        tags,
        scheduled,
        deadline,
        properties,
      ];
}
