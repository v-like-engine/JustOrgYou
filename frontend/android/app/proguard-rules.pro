# JustOrgYou ProGuard Rules for Security
# These rules protect the app from reverse engineering

## Flutter wrapper
-keep class io.flutter.app.** { *; }
-keep class io.flutter.plugin.** { *; }
-keep class io.flutter.util.** { *; }
-keep class io.flutter.view.** { *; }
-keep class io.flutter.** { *; }
-keep class io.flutter.plugins.** { *; }

## Hive database
-keep class hive.** { *; }
-keep class * extends hive.HiveObject { *; }

## Security: Obfuscate everything else
-repackageclasses 'o'
-allowaccessmodification
-optimizationpasses 5

## Remove logging in production
-assumenosideeffects class android.util.Log {
    public static *** d(...);
    public static *** v(...);
    public static *** i(...);
}

## Kotlin
-dontwarn kotlin.**
-keepclassmembers class **$WhenMappings {
    <fields>;
}

## Keep model classes for serialization
-keep class com.justorgyou.app.models.** { *; }

## Security: Remove stack traces
-keepattributes *Annotation*
-keepattributes SourceFile,LineNumberTable
-renamesourcefileattribute SourceFile
