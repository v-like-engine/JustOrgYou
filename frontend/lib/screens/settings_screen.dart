import 'package:flutter/material.dart';

class SettingsScreen extends StatelessWidget {
  const SettingsScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Settings'),
      ),
      body: ListView(
        children: [
          const ListTile(
            leading: Icon(Icons.info_outline),
            title: Text('JustOrgYou'),
            subtitle: Text('Version 0.1.0'),
          ),
          const Divider(),
          ListTile(
            leading: const Icon(Icons.upload_file),
            title: const Text('Import from Org Mode'),
            subtitle: const Text('Import tasks from .org files'),
            onTap: () {
              // TODO: Implement import
            },
          ),
          ListTile(
            leading: const Icon(Icons.download),
            title: const Text('Export to Org Mode'),
            subtitle: const Text('Export all tasks to .org file'),
            onTap: () {
              // TODO: Implement export
            },
          ),
          const Divider(),
          ListTile(
            leading: const Icon(Icons.cloud_upload),
            title: const Text('Sync Settings'),
            subtitle: const Text('Configure cloud synchronization'),
            onTap: () {
              // TODO: Implement sync settings
            },
          ),
          ListTile(
            leading: const Icon(Icons.smart_toy),
            title: const Text('AI Features'),
            subtitle: const Text('Configure AI-powered features'),
            onTap: () {
              // TODO: Implement AI settings
            },
          ),
          const Divider(),
          ListTile(
            leading: const Icon(Icons.palette),
            title: const Text('Appearance'),
            subtitle: const Text('Theme and display settings'),
            onTap: () {
              // TODO: Implement appearance settings
            },
          ),
          ListTile(
            leading: const Icon(Icons.notifications),
            title: const Text('Notifications'),
            subtitle: const Text('Manage notification settings'),
            onTap: () {
              // TODO: Implement notification settings
            },
          ),
        ],
      ),
    );
  }
}
