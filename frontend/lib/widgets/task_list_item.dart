import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_slidable/flutter_slidable.dart';
import '../models/task.dart';
import '../blocs/task_bloc.dart';

class TaskListItem extends StatelessWidget {
  final Task task;
  final bool showKeyword;

  const TaskListItem({
    super.key,
    required this.task,
    this.showKeyword = true,
  });

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 8),
      child: Slidable(
        endActionPane: ActionPane(
          motion: const ScrollMotion(),
          children: [
            SlidableAction(
              onPressed: (_) {
                context.read<TaskBloc>().add(DeleteTask(task.id));
              },
              backgroundColor: Colors.red,
              foregroundColor: Colors.white,
              icon: Icons.delete,
              label: 'Delete',
            ),
          ],
        ),
        child: Card(
          elevation: task.isOverdue ? 3 : 1,
          color: task.isOverdue ? Colors.red.shade50 : null,
          child: InkWell(
            onTap: () {
              // TODO: Show task details
            },
            borderRadius: BorderRadius.circular(12),
            child: Padding(
              padding: const EdgeInsets.all(12),
              child: Row(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  // Checkbox
                  Checkbox(
                    value: task.isDone,
                    onChanged: (_) {
                      context.read<TaskBloc>().add(ToggleTaskDone(task.id));
                    },
                  ),
                  const SizedBox(width: 8),

                  // Content
                  Expanded(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Row(
                          children: [
                            if (task.priority != null) ...[
                              _buildPriorityBadge(task.priority!),
                              const SizedBox(width: 8),
                            ],
                            if (showKeyword) ...[
                              _buildKeywordChip(task.keyword),
                              const SizedBox(width: 8),
                            ],
                          ],
                        ),
                        if (task.priority != null || showKeyword)
                          const SizedBox(height: 4),
                        Text(
                          task.title,
                          style: TextStyle(
                            fontSize: 16,
                            fontWeight: FontWeight.w500,
                            decoration: task.isDone
                                ? TextDecoration.lineThrough
                                : null,
                          ),
                        ),
                        if (task.body != null) ...[
                          const SizedBox(height: 4),
                          Text(
                            task.body!,
                            style: TextStyle(
                              fontSize: 14,
                              color: Colors.grey[600],
                            ),
                            maxLines: 2,
                            overflow: TextOverflow.ellipsis,
                          ),
                        ],
                        if (task.tags.isNotEmpty) ...[
                          const SizedBox(height: 8),
                          Wrap(
                            spacing: 4,
                            runSpacing: 4,
                            children: task.tags
                                .map((tag) => Chip(
                                      label: Text(
                                        tag,
                                        style: const TextStyle(fontSize: 12),
                                      ),
                                      visualDensity: VisualDensity.compact,
                                    ))
                                .toList(),
                          ),
                        ],
                        if (task.deadline != null || task.scheduled != null) ...[
                          const SizedBox(height: 8),
                          Row(
                            children: [
                              if (task.deadline != null) ...[
                                Icon(
                                  Icons.event,
                                  size: 16,
                                  color: task.isOverdue
                                      ? Colors.red
                                      : Colors.grey[600],
                                ),
                                const SizedBox(width: 4),
                                Text(
                                  _formatDate(task.deadline!),
                                  style: TextStyle(
                                    fontSize: 12,
                                    color: task.isOverdue
                                        ? Colors.red
                                        : Colors.grey[600],
                                  ),
                                ),
                                if (task.scheduled != null)
                                  const SizedBox(width: 12),
                              ],
                              if (task.scheduled != null) ...[
                                Icon(
                                  Icons.schedule,
                                  size: 16,
                                  color: Colors.grey[600],
                                ),
                                const SizedBox(width: 4),
                                Text(
                                  _formatDate(task.scheduled!),
                                  style: TextStyle(
                                    fontSize: 12,
                                    color: Colors.grey[600],
                                  ),
                                ),
                              ],
                            ],
                          ),
                        ],
                      ],
                    ),
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildPriorityBadge(int priority) {
    final letter = String.fromCharCode(65 + priority); // A, B, C, etc.
    final colors = [
      Colors.red,
      Colors.orange,
      Colors.yellow,
    ];
    final color = priority < colors.length ? colors[priority] : Colors.grey;

    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
      decoration: BoxDecoration(
        color: color.withOpacity(0.2),
        border: Border.all(color: color),
        borderRadius: BorderRadius.circular(4),
      ),
      child: Text(
        letter,
        style: TextStyle(
          fontSize: 12,
          fontWeight: FontWeight.bold,
          color: color.shade800,
        ),
      ),
    );
  }

  Widget _buildKeywordChip(String keyword) {
    final colors = {
      'TODO': Colors.blue,
      'DONE': Colors.green,
      'INBOX': Colors.orange,
      'WAITING': Colors.purple,
      'SOMEDAY': Colors.teal,
    };

    final color = colors[keyword] ?? Colors.grey;

    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
      decoration: BoxDecoration(
        color: color.withOpacity(0.2),
        borderRadius: BorderRadius.circular(12),
      ),
      child: Text(
        keyword,
        style: TextStyle(
          fontSize: 11,
          fontWeight: FontWeight.w600,
          color: color.shade800,
        ),
      ),
    );
  }

  String _formatDate(DateTime date) {
    final now = DateTime.now();
    final today = DateTime(now.year, now.month, now.day);
    final dateOnly = DateTime(date.year, date.month, date.day);

    if (dateOnly == today) {
      return 'Today';
    } else if (dateOnly == today.add(const Duration(days: 1))) {
      return 'Tomorrow';
    } else if (dateOnly == today.subtract(const Duration(days: 1))) {
      return 'Yesterday';
    } else {
      return '${date.day}/${date.month}/${date.year}';
    }
  }
}
